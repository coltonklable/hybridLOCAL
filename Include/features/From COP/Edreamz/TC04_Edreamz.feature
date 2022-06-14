Feature: TC04_Shop for Round-Trip in TS, Price, Book with multi PAX, Ticketing in UAPI and Partial ETicket Refund on SmartPoint

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
				| 31RB | RoundTrip | SYD-MEL-SYD |90,95 | 1-ADT,1-CHD | GDS || Permitted | QF ||| GBP |||| A12D96C8-EB29-4443-A27E-18A72C183E87 |
				
Scenario: "Price" transaction
Given UAPI Pricing transaction for 1G providercode
When AirItinerary has airsegments from tripservice SearchResponse
Then Pricing Transaction should be successfull

Scenario: "AirCreateReservation" transaction
Given uapi Booking transaction
When AirCreateReservationRequest has multi BookingTraveler and FormOfPayment details
And add AirPricingSolution from the pricing Response
Then AirCreateReservationRequest transaction should be successful

Scenario: "UniversalRecordRetrieve" transaction after booking
Given uapi UniversalRetrieve transaction 
When request has UniversalRecordlocatorCode from the AirCreate Response
Then UniversalRecordRetrieve transaction should be successfull

Scenario: "Ticketing" transaction
Given uapi Ticket transaction
When request has the AirReservationLocatorCode from the UniversalRecordRetrieve Response
Then Ticketing transaction should be successfull

Scenario: "UniversalRecordRetrieve" transaction after ticketing
Given uapi UniversalRetrieve transaction 
When request has UniversalRecordlocatorCode from the UniversalRecordRetrieve Response
Then UniversalRecordRetrieve transaction should be successfull
And should have Ticketing details

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
    	| country | PCC  |      Tab        | 	Customer |     Refund     |
      | LON     | 31RB  | TicketRefund   |  EDreams  | PARTIALREFUND  |