export class Animate {
  constructor(id) {
      console.log(`JS: on constructor (${id})`)
      this._id = id
      this._tween = TweenMax.to(`#${id}`, .3, { css:{ "filter": "drop-shadow(0 0 0.25rem black)"} })
      this._tween.pause()
  }

  enter() {
      console.log(`JS: on mouse enter (${this._id})`)
      this._tween.play()
  }

  leave() {
      console.log(`JS: on mouse leave (${this._id})`)
      this._tween.reverse()
  }

  destroy() {
    console.log(`JS: on destroy (${this._id})`)
    gsap.killTweensOf(`#${this._id}`)
    this._tween = undefined
    this._id = undefined
  }
}
