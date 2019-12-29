use std::rc::Rc;
use std::boxed::Box;
use std::cell::RefCell;
use yew::scheduler::{scheduler, Scheduler, Runnable};

#[allow(missing_debug_implementations)]
struct ImmediateRunnable {
    output: Rc<RefCell<Vec<i32>>>,
    value: i32,
}

impl Runnable for ImmediateRunnable {
    fn run(self: Box<Self>) {
        let mut output = self.output.borrow_mut();
        output.push(self.value);
    }
}

#[allow(missing_debug_implementations)]
struct IndirectionRunnable {
    output: Rc<RefCell<Vec<i32>>>,
    scheduler: Rc<Scheduler>,
}

impl Runnable for IndirectionRunnable {
    fn run(self: Box<Self>) {
        self.scheduler.put_and_try_run(Box::new(ImmediateRunnable {
            output: self.output.clone(),
            value: 1,
        }));
        self.output.borrow_mut().push(2);
        self.scheduler.put_and_try_run(Box::new(ImmediateRunnable {
            output: self.output.clone(),
            value: 3,
        }));
        self.output.borrow_mut().push(4);
        self.scheduler.put_and_try_run(Box::new(ImmediateRunnable {
            output: self.output.clone(),
            value: 5,
        }));
    }
}

#[test]
fn ordering() {
    let output = Rc::new(RefCell::new(Vec::<i32>::new()));
    let sched = scheduler();
    sched.put_and_try_run(Box::new(IndirectionRunnable {
        output: output.clone(),
        scheduler: sched.clone(),
    }));

    let output_result = output.borrow();

    assert_eq!(output_result[0], 2);
    assert_eq!(output_result[1], 4);
    assert_eq!(output_result[2], 1);
    assert_eq!(output_result[3], 3);
    assert_eq!(output_result[4], 5);
}
