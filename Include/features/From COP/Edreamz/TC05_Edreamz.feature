Feature: TC05_Shop for RoundTrip in TS Price, Book & Ticketing in UAPI and Void Ticket on SmartPoint

Scenario Outline:  "Search Service" transaction
Given OAUTH Token for PCC <PCC> is generated 
And the Trip Type is <TripType> OD is <FromTo> Departure Date is <DepartureDates> Pax Type is <PaxType>
And the Content Source is <ContentSource> ReturnBrandedFaresInd is <returnBrandedFaresInd>
When the Search Service with Access Group as <AccessGroup> is invoked
Then Validate Search Response
And RoundTrip segments
	Examples:
	      | PCC  | TripType  | FromTo  		 | DepartureDates   | PaxType 			| ContentSource | AccessGroup | returnBrandedFaresInd |
				| 31RB | RoundTrip | ATL-DEN-ATL | 80,90 						| 1-ADT 	| GDS 					| A12D96C8-EB29-4443-A27E-18A72C183E87 ||
				
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
When request has AiReservationLocatorCode from the UniversalRecordRetrieveResponse
Then AirTicketing should be successful

Scenario Outline: Post ticketing - Void ETicket
    Given Launch Smartpoint
    ## And Change Application settings to <Core>
    ##And Login Smartpoint
    And Change the currency specific to <country> 
    And Emulate the <PCC>
    And Retrieve the ticketed PNR
    And Launch SAT
    And Navigate to SAT <Tab>
    When Select Void Check box for Passenger <Paxname> and Segment <Segment>
    And Perform Void Operations
    Then Validate Ticket details removed from SAT Screen
    And Close SAT
    And Validate Tickets status for Void
    And Close SmartPoint

    Examples: 
      | country | PCC  | Tab  |Paxname	| Segment |
      | LON     | 31RB | Void |   1    |		 1		|
     
    
     

