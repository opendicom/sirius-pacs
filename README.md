# sirius-pacs

pacs and viewer html5 dckv j2h (DICOM contextual key value) written in rust. sirius-pacs is responsible to serve to users studies already organized in a storage and indexed in a table where the columns are matching fields for study selection. sirius-pacs does not agregate new studies to the storage and index. This task is reserved to sirius-store.

dckv is described at length in https://github.com/jacquesfauquex/DCKV

j2h is the high throughput protocol of jpeg 2000

Detailed documentation:
- why DCKV instead of DICOMweb ?
- A restriction of IHE IID as a starting point for the html5 viewer API
- ...
