Feature: TC03_Shop for Round-Trip in TS Price Book with multi PAX Ticketing AMOA AMF in UAPI and Even Exchange on SmartPoint

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
	      | PCC  | TripType  | FromTo      | DepartureDates | PaxType           | ContentSource | returnBrandedFaresInd | CarrierPreferenceType | Carrier | CabinPreferenceType | Cabin | CurrencyCode | ProhibitMaxStayFaresInd | ProhibitAdvancePurchaseFaresInd | FareType | AccessGroup                          |
				| 31RB | RoundTrip | MAD-ZRH-MAD | 80,85            | 1-ADT,1-CHD | GDS 					|												|												|					|											|				| USD 				 |												 |                                 |          | A12D96C8-EB29-4443-A27E-18A72C183E87 |
				
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

Scenario: "AirMerchandisingOfferAvailability" transaction 
Given uapi AirMerchandisingOfferAvailability transaction
When request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response 
Then AirMerchandisingOfferAvailability should be successful
 
Scenario: "AirMerchandisingFulfillment" transaction
Given uapi AirMerchandisingFulfillment transaction
When request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response 
And add SearchTraveler, segment, OptionalServices from AirMerchandisingOfferAvailability response
Then AirMerchandisingFulfillment should be successful

Scenario Outline: Post ticketing flow
Given Launch Smartpoint
#And Login Smartpoint
And Change the currency specific to <country> 
And Emulate the <PCC>
And Retrieve the ticketed PNR
When Change the <Date> of travel for <Segment>
And Cancel Fares and add new Fares for the PNR
And Launch SAT
And Navigate to SAT <Tab>
And Check the difference in Price
And Perform Voluntary exchange Operations for <Customer>
Then Validate Ticket status
And Close SmartPoint

 Examples: 
      | country | PCC  |      Tab        | 	PNR			| Date  | Segment |
      | LON     | 31RB | VoluntaryChange | *18832D  | 02Aug |		 1		|
      #| LON     | 5ou3 | VoluntaryChange | *254789  | 16Jun |		 2		|