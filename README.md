# pawchop
Like porechop but implemented in rust - not an official project, yet



### Defining a universe of kits and associated adapters and barcodes

The pawchop software is built containing a set of adapters, primers, and sequence
information from recent sequencing kits. The software can also accommodate the
usage of third party adapter and primer information. This information should
be imported from a JSON format file that provides information as per the
example below

```
{
    "document_title": "Chemistry Technical Document",
    "document_authors": "Oxford Nanopore Technologies",
    "source": "https://community.nanoporetech.com/technical_documents/chemistry-technical-document/v/chtd_500_v1_revai_07jul2016",
    "date_copied": "Jan 11 2023",
    "sequencing_kits": [
        {
            "kit_name": "Demo Sequencing Kit - dummy information",
            "kit_code": "ILLUSTRATION_ONLY",
            "barcodes": [
                {
                    "component": "BC01",
                    "forward_sequence": "AAGAAAGTTGTCGGTGTCTTTGTG",
                    "reverse_sequence": null
                },
                {
                    "component": "BC02",
                    "forward_sequence": "TCGATTCCGTTTGTAGTCGTCTGT",
                    "reverse_sequence": null
                }
            ],
            "adapters": [
                {
                    "adapter_name": "Ligation Adapter",
                    "adapter_code": "LA",
                    "top_strand": "5'-TTTTTTTTCCTGTACTTCGTTCAGTTACGTATTGCT-3'",
                    "bottom_strand": null
                }
            ]
        }
    ]
}
```
