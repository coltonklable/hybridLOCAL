Feature: TC08_Shop for RoundTrip in TS Price, Book & Ticketing in UAPI and Manual Full Refund Ticket on SmartPoint

Scenario Outline:  "Search Service" transaction
Given OAUTH Token for PCC <PCC> is generated 
And the Trip Type is <TripType> OD is <FromTo> Departure Date is <DepartureDates> Pax Type is <PaxType>
And the Content Source is <ContentSource> ReturnBrandedFaresInd is <returnBrandedFaresInd>
And the Cabin Preference Type is <CabinPreferenceType> Cabin is <Cabin>
When the Search Service with Access Group as <AccessGroup> is invoked
Then Validate Search Response
And RoundTrip segments
	Examples:
	      | PCC  | TripType  | FromTo  		 | DepartureDates   | PaxType 			| ContentSource |CabinPreferenceType | Cabin  | AccessGroup                         | returnBrandedFaresInd |
				| 31RB | RoundTrip | ADL-ORD-ADL | 80,90						| 1-ADT 	| GDS 					| 					 ||A12D96C8-EB29-4443-A27E-18A72C183E87 ||
				
Scenario: "Price" transaction             
Given  UAPI AirPrice transaction for 1G providercode
When  AirItinerary has segments from tripservice SearchResponse
Then AirPrice Transaction should be successful  

Scenario: "AirBook" transaction  
Given uapi AirBook transaction
When AirBook Request has BookingTraveler and FormOfPayment details
And add AirPricingSolution from the airpricing Response
Then AirBook should be successful   

Scenario: "URRetrieve" transaction
Given uapi URRetrieve transaction 
When request has UniversalRecordlocatorCode from the AirBook Response
Then URRetrieve should be successful

Scenario: "AirTicketing" transaction
Given uapi AirTicketing transaction
When request has AiReservationLocatorCode from the URRetrieve Response
Then AirTicketing should be successful  
  
Scenario Outline: Post ticketing - Manual Full Refund
   Given Launch Smartpoint
   #And Login Smartpoint
   #And Change the currency to the currency of PCC CITY 
   And Emulate the <PCC>
   And Retrieve the ticketed PNR
   And Validate Tickets status for ticketed PNR
   And Perform Manual Full Refund Operation <Date>  
   Then Validate Tickets status as RFND in *HTE Response
   And Close SmartPoint


    Examples: 
      | country | PCC  |   Date    | 
      | LON     | 31RB |  20MAY22  | 
  