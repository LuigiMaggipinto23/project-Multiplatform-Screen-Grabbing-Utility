# screen_grabbing_utility
Quando sono collegati più schermi, viene aperto nella terza pagina solo lo screen di uno dei due schermi, non entrambi

DONE-> ho cambiato alcune cose nella struct FirstWindow. Ora invece di salvare una singola immagine, salviamo un vettore di immagini
che sono quelle prese da ciascuno schermo facendo lo screen. Inoltre, invece di salvare un singolo path, salviamo un vettore di path.
Quindi nella posizione 'i-esima' del vettore screenshots_taken c'è l'i-esimo screen ed è associato all'i-esimo path salvato nel vettore fp.
In questo modo possiamo salvare più immagini e più path. Per i path ho usato una variabile 'i' che varia da 0 fino alla lunghezza del vettore
di screen che viene aggiunta al path iniziale (esempio: path iniziale: "masci/ao.png", poi faccio 3 screen e questi saranno salvati in "masci0/ao.png", "masci1/ao.png", "masci2/ao.png" ). 
!! Rimane il problema della visualizzazione delle immagini, continua ad essere mostrata ancora solo l'ultima "scattata" però, dato che gestiamo tutto con due vettori e un indice 'i', possiamo decidere noi quale è l'i-esima immagine da mostrare.

Aggiungere funzionamento tasti 
DONE-> Nel main ho creato un keyEventManager e le due shortcut: ESC (per aprire la first window) e CTRL+D (per aprire la finestra opaca sulla quale fare lo screen rettangolare oppure per fare lo screen all'intera pagina). Ho registrato le due chiavi nel manager e poi ho creato un Receiver (openfw)che si occuperà di capire quando vengono premuti e rilasciati i tasti. Inizialmente avevo provato con due diversi Receiver, uno per ciascun tasto ma si sovrapponevano e non funzionava. Alla fine ho risolto usando un solo Receiver e poi gestendo tutto con un match nella update. Praticamente il Receiver associa un 'id' ad ogni tasto premuto e quelli che vedete nel match (2439345500 e 2440410256) sono gli id associati alla pressione del tasto ESC e del tasto CTRL+D rispettivamente. Quindi in base a questi id, decido se mostrare la window 1 oppure la 2. Su Microsoft funziona perfettamente.
!! Forse si può ottimizzare gestione event.id

Aggiungere post-processing


Screen grabbing utility 

Cose da fare per il progetto:
- [ ] Ritaglio immagine in post processing;
- [x] Gestione più monitor;
    - [ ] Testare su Mac 
    - [ ] Testare su più schermi
    - [ ] Risolvere 'problema' risoluzione diversa dal 100%

- [x] Finestra Setting , con casella testo per scegliere path (e path di default) e radio button per scegliere formato , salvataggio;
- [ ] Capire come salvare immagine con edit senza fare screen;

Aggiustamenti:
- [ ] Rendering shapes in tempo reale e evitare sfasamenti;
- [ ] Numeri fissi a cazzo per dimensioni finestra, prendendo dimensioni finestra da frame;
- [x] Cambiare puntatori del mouse nelle varie fasi (rettandgolo ,testo)
- [] Sistemare scrittura testo (posizionamento, dimensione a scelta) 
- [ ] Capire come funziona la selected window di buffer
- [x] Aggiungere testo : “premere ctrl+D per fare screen” 


Modulare:
- [x]Take_Screenshot: funzione a cui delegare la capture. (Width,height,current_os,rect_pos ,….)  
- [ ]Save_Screenot:  solo per fare il save;
- [x]Define_Rectangle: definire rettangolo  della mouse_pos ( da mettere dopo mouse_pos = ui.input) , da diff_x e diff_y 
- [ ]Init: Funzione Central panel select window 1 da delegare.
- [x]Refactor_on_windows: da chiamare negli if current_os==windows, modifica il self height e with; 
- [x]Load_image : Funzione per caricare immagine. 

