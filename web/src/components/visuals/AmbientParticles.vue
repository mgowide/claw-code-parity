<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import * as THREE from 'three'

const canvas = ref<HTMLCanvasElement | null>(null)

let renderer: THREE.WebGLRenderer
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let particles: THREE.Points
let animId: number

function init(el: HTMLCanvasElement) {
  renderer = new THREE.WebGLRenderer({ canvas: el, alpha: true, antialias: false })
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.setSize(el.clientWidth, el.clientHeight)
  renderer.setClearColor(0x000000, 0)

  scene = new THREE.Scene()
  camera = new THREE.PerspectiveCamera(60, el.clientWidth / el.clientHeight, 0.1, 1000)
  camera.position.z = 80

  // Build particle cloud
  const count = 800
  const positions = new Float32Array(count * 3)
  const colors = new Float32Array(count * 3)
  const palette = [
    new THREE.Color(0x58a6ff),
    new THREE.Color(0x3fb950),
    new THREE.Color(0xbc8cff),
    new THREE.Color(0xd29922),
  ]
  for (let i = 0; i < count; i++) {
    positions[i * 3] = (Math.random() - 0.5) * 160
    positions[i * 3 + 1] = (Math.random() - 0.5) * 120
    positions[i * 3 + 2] = (Math.random() - 0.5) * 80
    const c = palette[Math.floor(Math.random() * palette.length)]
    colors[i * 3] = c.r
    colors[i * 3 + 1] = c.g
    colors[i * 3 + 2] = c.b
  }

  const geo = new THREE.BufferGeometry()
  geo.setAttribute('position', new THREE.BufferAttribute(positions, 3))
  geo.setAttribute('color', new THREE.BufferAttribute(colors, 3))

  const mat = new THREE.PointsMaterial({
    size: 0.6,
    vertexColors: true,
    transparent: true,
    opacity: 0.55,
    sizeAttenuation: true,
  })

  particles = new THREE.Points(geo, mat)
  scene.add(particles)
}

function animate() {
  animId = requestAnimationFrame(animate)
  if (particles) {
    particles.rotation.y += 0.0006
    particles.rotation.x += 0.0002
  }
  renderer?.render(scene, camera)
}

function handleResize() {
  if (!canvas.value || !renderer || !camera) return
  const w = canvas.value.clientWidth
  const h = canvas.value.clientHeight
  camera.aspect = w / h
  camera.updateProjectionMatrix()
  renderer.setSize(w, h)
}

onMounted(() => {
  if (!canvas.value) return
  init(canvas.value)
  animate()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  cancelAnimationFrame(animId)
  renderer?.dispose()
  window.removeEventListener('resize', handleResize)
})
</script>

<template>
  <canvas
    ref="canvas"
    class="pointer-events-none absolute inset-0 h-full w-full"
  />
</template>
