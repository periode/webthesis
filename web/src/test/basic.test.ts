import { tick } from 'svelte'
import { afterEach, expect, test } from 'vitest'
import Date from '../routes/components/front/Date.svelte'
import TableOfContents from '../routes/components/TableOfContents.svelte'
import toc_data from '../data/toc.json'
import front_data from '../data/front.json'
import introduction_data from '../data/introduction.json'
import type { INode, IToCNode } from '../utils/types'
import Section from '../routes/components/Section.svelte'

let host: HTMLElement

const toc = toc_data as Array<IToCNode>;
const front = front_data as Array<INode>;
const introduction = introduction_data as INode;

afterEach(() => {
  host.remove()
})

test('mount date component', async () => {
  host = document.createElement('div')
  host.setAttribute('id', 'host')
  document.body.appendChild(host)
  const instance = new Date({ target: host, props: { date: "1678367692" } })
  expect(instance).toBeTruthy()
  expect(host.innerHTML).toContain('last updated on: 09.03.2023 at 14:14')
})

test('mount toc component', async () => {
  host = document.createElement('div')
  host.setAttribute('id', 'host')
  document.body.appendChild(host)
  const instance = new TableOfContents({ target: host, props: { toc: toc } })
  expect(instance).toBeTruthy()
  const intro = document.getElementById("chap:introduction")
  expect(intro?.children[0].children[0].innerHTML).toContain("Introduction")
})

test('mount section component', async () => {
  host = document.createElement('div')
  host.setAttribute('id', 'host')
  document.body.appendChild(host)
  const instance = new Section({ target: host, props: { nodes: [introduction] } })
  expect(instance).toBeTruthy()
})