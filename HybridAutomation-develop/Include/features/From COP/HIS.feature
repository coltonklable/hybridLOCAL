Feature:TC01_Shop in UAPI(LI2) and Price ,Book(8Q8) & ticket(8MK) on GWS

Scenario: Preconditions
Given Set manual proxy and Dates
Then manual proxy and dates should be set


Scenario: "LFS" transaction
Given UAPI LFS transaction for 1G provider
When the itinerary has oneway segment
And add LI2 PCC credentials
Then LFS Transaction should be successful

Scenario: "BeginSession" transaction for 8Q8
Given GWS BeginSession transaction for 1G provider
When BeginSession has 8Q8 Profile details
Then BeginSession Transaction should be successful
     
Scenario: "PNRBFManagement_49" transaction with out fare
Given GWS PNRBFManagement_49 transaction for booking air segments
When PNRBFManagement_49 Transaction has carrier,FlightNumber,Origin,destination details from UAPI LowFareSearch Response 
Then PNRBFManagement_49 Transaction should be successful

Scenario: "PNRBFManagement_49" transaction for PNRBFRetrieveMods
Given GWS PNRBFManagement_49 transaction for PNRBFRetrieveMods
When PNRBFManagement_49 Transaction has RecLoc from previous PNRBFManagement_49 Response
Then PNRBFManagement_49 Transaction should be successful for PNRBFRetrieveMods

Scenario: "FareQuoteStandard_32" transaction
Given GWS FareQuoteStandard_32 transaction
Then FareQuoteStandard_32 Transaction should be successful

Scenario: "PNRBFManagement_53" transaction for PNRBFRetrieveMods after fare Quote
Given GWS PNRBFManagement_53 transaction for PNRBFRetrieveMods
When PNRBFManagement_53 Transaction has RecLoc from PNRBFManagement_49 Response
Then PNRBFManagement_53 Transaction should be successful

Scenario: "PNRBFManagement_53" transaction for StorePriceMods
Given GWS PNRBFManagement_53 transaction for StorePriceMods
When PNRBFManagement_53 Transaction has carrier from UAPI LowFareSearch Response 
Then PNRBFManagement_53 Transaction should be successful for StorePriceMods

Scenario: "PNRBFManagement_53" transaction for PNRBFRetrieveMods after stored fare
Given GWS PNRBFManagement_53 transaction for PNRBFRetrieveMods
When PNRBFManagement_53 Transaction has RecLoc from PNRBFManagement_49 Response
Then PNRBFManagement_53 Transaction should be successful PNRBFRetrieveMods after stored fare

Scenario: "EndSession" transaction for 8Q8 PCC
When request has session key from BeginSession Response
Then EndSession Transaction should be successful

Scenario: "BeginSession" transaction for 8MK PCC
Given GWS BeginSession transaction for 1G provider for 8MK PCC
When BeginSession with 8MK Profile details
Then BeginSession Transaction should be successful for 8MK PCC

Scenario: "PNRBFManagement_53" transaction for PNRBFRetrieveMods for  8MK PCC
Given GWS PNRBFManagement_53 transaction for PNRBFRetrieveMods
When PNRBFManagement_53 Transaction has RecLoc from PNRBFManagement_49 Response
Then PNRBFManagement_53 Transaction should be successful2
                
Scenario: "DocProdFareManipulation_27" transaction
Given DocProdFareManipulation_27 transaction for 1G provider
When request has TransType as TK in TicketingControl
Then the DocProdFareManipulation_27 should be successful

Scenario: "EndSession" transaction for 8MK PCC
Given GWS EndSession transaction
When request has session key from BeginSession Response
Then EndSession Transaction should be successful2