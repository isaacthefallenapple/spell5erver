const rudeButton = document.querySelector("#post-button") as HTMLButtonElement

type Filters = {
    [filter: string]: Set<String>
    classes: Set<String>
    schools: Set<String>
    components: Set<String>
}

let filters: Filters = {
    classes: new Set(),
    schools: new Set(),
    components: new Set(),
}

const toJson = (f: Filters): string => {
    let stringifyable: {
        classes: String[]
        schools: String[]
        components: String[]
    } = {
        classes: [...f.classes],
        schools: [...f.schools],
        components: [...f.components],
    }

    return JSON.stringify(stringifyable)
}

const checkboxFilterLogic = (id: string) => {
    let filterSet = filters[id]
    ;(document.querySelectorAll(`input[id^="filter-${id}-"]`) as NodeListOf<
        HTMLInputElement
    >).forEach((el) => {
        filterSet.add(el.value)
        el.addEventListener("change", (e) => {
            if (el.checked) {
                filterSet.add(el.value)
            } else {
                filterSet.delete(el.value)
            }
            console.log(filterSet)
        })
    })
}

;["classes", "schools", "components"].forEach((id) => {
    checkboxFilterLogic(id)
})

const filterSetClasses = document.querySelector("#filter-set-classes")!
const filterInputs = filterSetClasses.querySelectorAll(
    "input[type='checkbox']",
) as NodeListOf<HTMLInputElement>
filterInputs.forEach((el) => {
    el.addEventListener("click", (e) => {
        const changeEvent = new Event("change")
        if (e.shiftKey) {
            if (!el.checked) {
                el.checked = true
                // el.dispatchEvent(changeEvent)
            }
            filterInputs.forEach((innerEl) => {
                if (innerEl.id !== el.id && innerEl.checked) {
                    innerEl.checked = false
                    innerEl.dispatchEvent(changeEvent)
                }
            })
        }
    })
})

rudeButton.addEventListener("click", () => {
    console.log("clicked!")
    const query = document.querySelector("input[name='q']")!.textContent
    fetch("/postjson", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            query: query,
            filters: toJson(filters),
        }),
    }).then((r) => console.log(r.status))
})
