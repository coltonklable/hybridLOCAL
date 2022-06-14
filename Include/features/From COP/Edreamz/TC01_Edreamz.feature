Feature: TC01_Shop for one-way  in TS Price Book and Ticketing in UAPI and ETICKET EXCHANGE (ACM) on SmartPoint
##


Scenario Outline: abc
Given OAUTH Token for PCC <PCC> is generated 
And the Trip Type is <TripType> OD is <FromTo> Departure Date is <DepartureDates> Pax Type is <PaxType>
And the Content Source is <ContentSource> ReturnBrandedFaresInd is <returnBrandedFaresInd>
And the Carrier Preference Type is <CarrierPreferenceType> Carrier is <Carrier>
And the Cabin Preference Type is <CabinPreferenceType> Cabin is <Cabin>
And the Currency Code is <CurrencyCode> ProhibitMaxStayFaresInd is <ProhibitMaxStayFaresInd> ProhibitAdvancePurchaseFaresInd is <ProhibitAdvancePurchaseFaresInd> Fare Type is <FareType>
When the Search Service with Access Group as <AccessGroup> is invoked
Then Validate Search Response
	Examples:
	      | PCC | TripType | FromTo | DepartureDates | PaxType | ContentSource | returnBrandedFaresInd | CarrierPreferenceType | Carrier | CabinPreferenceType | Cabin | CurrencyCode | ProhibitMaxStayFaresInd | ProhibitAdvancePurchaseFaresInd | FareType | AccessGroup |
				| 31RB | Oneway | DEN-ORD | 90              | 1-ADT  | GDS || Permitted | UA | Permitted | Business | AUD |||| A12D96C8-EB29-4443-A27E-18A72C183E87 |
				
Scenario: "Price" transaction             
Given  UAPI Price transaction for 1G provider
When  AirItinerary has air segments from tripservice SearchResponse
Then Price Transaction should be successful

Scenario: "AirCreateReservation" transaction  
Given uapi AirCreateReservation transaction
When AirCreateReservationReq has BookingTraveler and FormOfPayment details
And add PricingSolution from the pricing Response
Then AirCreateReservation should be successful

Scenario: "UniversalRecordRetrieve" transaction
Given uapi UniversalRecordRetrieve transaction 
When request has UniversalRecordlocatorCode from the AirCreateReservation Response
Then UniversalRecordRetrieve should be successful

Scenario: "Ticketing" transaction
Given uapi Ticketing transaction
When request has AiReservationLocatorCode from the UniversalRecordRetrieve Response
Then Ticketing should be successful


Scenario Outline: Post ticketing - Voluntary Exchange ACM Exchange flow
    Given Launch Smartpoint
    # And Change Application settings to <Core>
    #And Login Smartpoint
    And Change the currency specific to <country>
    And Emulate the <PCC>
    And Retrieve the ticketed PNR
    When Change the <Class> of travel for <Segment>
    And Cancel Fares and add new Fares for the PNR
    And Launch SAT
    And Navigate to SAT <Tab>
    And Check the difference in Price
    And Perform Voluntary exchange Operations for <Customer>
    Then Validate Ticket status
    And Close SmartPoint

    Examples: 
      | country | PCC   |      Tab        |Segment |  Class   |Customer|
      | LON     | 31RB  | VoluntaryChange |   1	   |    B     |			Edreamz|