Feature: TC06_Shop for RoundTrip  in TS, Price, Book, Ticketing, AMOA, SeatMap & AMF in UAPI and Exchange(EMDRB) on SmartPoint

Scenario Outline:  "Search Service" transaction
Given OAUTH Token for PCC <PCC> is generated 
And the Trip Type is <TripType> OD is <FromTo> Departure Date is <DepartureDates> Pax Type is <PaxType>
And the Content Source is <ContentSource> ReturnBrandedFaresInd is <returnBrandedFaresInd>
And the Carrier Preference Type is <CarrierPreferenceType> Carrier is <Carrier>
And the Cabin Preference Type is <CabinPreferenceType> Cabin is <Cabin>
When the Search Service with Access Group as <AccessGroup> is invoked
Then Validate Search Response
And RoundTrip segments
	Examples:
	      | PCC  | TripType  | FromTo  		 | DepartureDates   | PaxType 			| ContentSource |CabinPreferenceType | Cabin  | CarrierPreferenceType | Carrier | AccessGroup                         | returnBrandedFaresInd |
				| 31RB | RoundTrip | LHR-AMS-LHR | 80,90						| 1-ADT 	| GDS 					| Permitted					 |Business|					Permitted			|	BA		|A12D96C8-EB29-4443-A27E-18A72C183E87 ||
			
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

Scenario: "UniversalRecordRetrieve" transaction after ticketing
Given uapi UniversalRetrieve transaction 
When request has UniversalRecordlocatorCode from the UniversalRecordRetrieve Response
Then UniversalRecordRetrieve transaction should be successfull
And should have Ticketing details

Scenario: "AirMerchandisingOfferAvailability" transaction 
Given uapi AirMerchandisingOfferAvailability transaction
When request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response 
Then AirMerchandisingOfferAvailability should be successful
 
Scenario: "AirMerchandisingFulfillment" transaction
Given uapi AirMerchandisingFulfillment transaction
When request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response 
And add SearchTraveler, segment, OptionalServices from AirMerchandisingOfferAvailability response
Then AirMerchandisingFulfillment should be successful

Scenario: "SeatMap" transaction
Given uapi SeatMap transaction
When request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response
Then SeatMap should be successful

Scenario: "AirMerchandisingFulfillment" transaction
Given uapi AirMerchandisingFulfillment transaction
When request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response 
And add SearchTraveler, segment, OptionalServices from AirMerchandisingOfferAvailability response
Then AirMerchandisingFulfillment should be successful

Scenario Outline: Post ticketing - Voluntary Exchange EMDRB Exchange flow
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
      | country | PCC   |      Tab        |Segment |  Class   |
      | LON     | 31RB  | VoluntaryChange |   1	   |    J     |