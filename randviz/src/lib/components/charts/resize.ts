import { NEVER, Observable, Subject, fromEvent } from "rxjs";
import { filter, map, pairwise, startWith, switchMap } from "rxjs/operators";

const resizable = <T extends HTMLElement>(elem: T) => {
  return new Observable<T>((subscriber) => {
    let ro = new ResizeObserver(() => {
      subscriber.next(elem);
    });

    ro.observe(elem);

    return () => {
      ro.disconnect();
    };
  });
};

export { resizable };


const onHeightChange = <T extends HTMLElement>(initialHeight: number = 0) => {
    let subject = new Subject<T | null>();
  
    let status = true;
  
    let trigger = (elem: T | null) => {
      subject.next(elem);
    };
  
    let enable = () => {
      status = true;
    };
  
    let disable = () => {
      status = false;
    };
  
    let state$ = subject.asObservable().pipe(
      switchMap((elem) => {
        if (!elem) return NEVER;
        return resizable(elem);
      }),
      filter(() => status),
      map((target) => target.offsetHeight),
      startWith(initialHeight)
    );
  
    return {
      state$,
      trigger,
      enable,
      disable
    };
  };
  


  let form = document.getElementById("form")! as HTMLFormElement;
  let content = document.getElementById("text-content")!;
  
  let setText = (text: string) => {
    content.textContent = text;
  };
  
  let getData = () => {
    let formData = new FormData(form);
    let status = formData.get("status");
    let trigger = formData.get("trigger") as string;
  
    return {
      status,
      trigger
    };
  };
  
  let handler = onHeightChange();
  
  handler.state$
    .pipe(
      pairwise(),
      filter(([a, b]) => a < 300 || b < 300),
      map(([a, b]) => Math.min(b, 300))
    )
    .subscribe((height) => {
      setText(`height is ${height}`);
    });
  
  fromEvent(form, "change")
    .pipe(startWith(null))
    .subscribe(() => {
      let data = getData();
  
      if (data.status === null) {
        handler.disable();
      } else {
        handler.enable();
      }
  
      let trigger = document.getElementById(`trigger-${data.trigger}`);
  
      handler.trigger(trigger);
    });
  