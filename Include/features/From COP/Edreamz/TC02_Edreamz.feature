Feature: TC02_Shop for Round-Trip in TS Price Book and Ticketing in UAPI and Full ETicket Refund on SmartPoint

Scenario Outline: abc
Given OAUTH Token for PCC <PCC> is generated 
And the Trip Type is <TripType> OD is <FromTo> Departure Date is <DepartureDates> Pax Type is <PaxType>
And the Content Source is <ContentSource> ReturnBrandedFaresInd is <returnBrandedFaresInd>
And the Carrier Preference Type is <CarrierPreferenceType> Carrier is <Carrier>
And the Cabin Preference Type is <CabinPreferenceType> Cabin is <Cabin>
And the Currency Code is <CurrencyCode> ProhibitMaxStayFaresInd is <ProhibitMaxStayFaresInd> ProhibitAdvancePurchaseFaresInd is <ProhibitAdvancePurchaseFaresInd> Fare Type is <FareType>
When the Search Service with Access Group as <AccessGroup> is invoked
Then Validate Search Response
And RoundTrip segments
	Examples:
	      | PCC | TripType | FromTo | DepartureDates | PaxType | ContentSource | returnBrandedFaresInd | CarrierPreferenceType | Carrier | CabinPreferenceType | Cabin | CurrencyCode | ProhibitMaxStayFaresInd | ProhibitAdvancePurchaseFaresInd | FareType | AccessGroup |
				| 31RB | RoundTrip | DEN-ATL-DEN | 80,90 | 1-ADT | GDS |||||| USD |||| A12D96C8-EB29-4443-A27E-18A72C183E87 |
				
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

Scenario Outline: Post ticketing - Refund flow
    Given Launch Smartpoint
    # And Change Application settings to <Core>
    #And Login Smartpoint
    And Change the currency specific to <country>
    And Emulate the <PCC>
    And Retrieve the ticketed PNR
    And Launch SAT
    And Navigate to SAT <Tab>
    And Validate the Retrieved ticket number and Status
    When Perform <Refund> Operation for <Customer>
    Then Validate Refunded Ticket status
    And Close SmartPoint
    
    Examples:
    	| country | PCC  |      Tab        | Customer |     Refund     |
      | LON     | 31RB  | TicketRefund   | EDreams  |   FULLREFUND   |
  
    
    

    