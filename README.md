# sirius-pacs

sirius-pacs is the server used by sirius-view on the client side. 

sirius-pacs is responsible to serve studies as fast as posible to the html5 viewer.

sirius-pacs does not agregate new studies. This task is reserved to sirius-store.

sirius-pacs uses dicom representation eDCKV (DCKV with series+instance prefix). 
DCKV (DICOM Contextual Key Value) and eDCKV are described at length in https://github.com/jacquesfauquex/DCKV

sirius-pacs uses pixel compression j2k 4 quality layers on the storage side and j2h (high throughput protocol of jpeg 2000) to comunicate with sirius-view. This implies that sirius-pacs includes a transcoder j2k>j2h

Detailed documentation of sirius-pacs in this [wiki](https://github.com/opendicom/sirius-pacs/wiki) and of sirius-view in this other [wiki](https://github.com/opendicom/sirius-view/wiki)
