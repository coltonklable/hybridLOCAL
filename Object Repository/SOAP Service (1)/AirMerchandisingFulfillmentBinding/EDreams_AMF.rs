<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>EDreams_AMF</name>
   <tag></tag>
   <elementGuidId>223a56f5-bca9-4626-9856-346ecd04c13f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://localhost:8080/kestrel/AirService</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;soapenv:Body>
      &lt;univ:AirMerchandisingFulfillmentReq TargetBranch=&quot;P7113930&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:common_v50_0=&quot;http://www.travelport.com/schema/common_v50_0&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
         &lt;!--&lt;common_v50_0:HostToken Key=&quot;b4a9qZAZ3BKA+LKAAAAAAA==&quot;>H4sIAAAAAAAAAFvzloG1hIE11NEn1BEAkEkEtAwAAAA={IS@@@}H4sIAAAAAAAAAFvzloG1hIE5NNgFAE5M+cEKAAAA{CC@@@ET}ACHSDv01LPD1:66f03099-9e29-4569-80fa-859c9176b294&lt;/common_v50_0:HostToken>-->
         &lt;!--&lt;common_v50_0:HostToken Key=&quot;b4a9qZAZ3BKABMKAAAAAAA==&quot;>H4sIAAAAAAAAAFvzloG1hIE11NEn1BEAkEkEtAwAAAA={IS@@@}H4sIAAAAAAAAAFvzloG1hIE5NNgFAE5M+cEKAAAA{CC@@@ET}ACHSDv01LPD1:f68e9bc4-0648-45ea-a861-f9a10a627992&lt;/common_v50_0:HostToken>-->
         &lt;air:HostReservation Carrier=&quot;KL&quot; CarrierLocatorCode=&quot;S9I4DN&quot; ProviderCode=&quot;1G&quot; ProviderLocatorCode=&quot;0CXQV4&quot;/>
         &lt;air:AirSolution>
            &lt;air:SearchTraveler Key=&quot;8nfO9ysY1DKASJUHAAAAAA==&quot; Code=&quot;ADT&quot;>
               &lt;common_v50_0:Name First=&quot;Abraham Lincoln&quot; Last=&quot;Martin Luther King&quot;/>
            &lt;/air:SearchTraveler>
            &lt;air:AirSegment Key=&quot;TLsU9ytY1DKAMGdHAAAAAA==&quot; Group=&quot;0&quot; Carrier=&quot;KL&quot; CabinClass=&quot;Economy&quot; FlightNumber=&quot;1673&quot; ProviderCode=&quot;1G&quot; Origin=&quot;AMS&quot; Destination=&quot;BCN&quot; DepartureTime=&quot;2021-12-10T14:25:00.000+01:00&quot; ArrivalTime=&quot;2021-12-10T16:35:00.000+01:00&quot; TravelTime=&quot;130&quot; Distance=&quot;771&quot; ClassOfService=&quot;G&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;73H&quot; Status=&quot;HK&quot; ChangeOfPlane=&quot;false&quot; GuaranteedPaymentCarrier=&quot;No&quot; ProviderReservationInfoRef=&quot;8nfO9ysY1DKAvKUHAAAAAA==&quot; ProviderSegmentOrder=&quot;1&quot; OptionalServicesIndicator=&quot;true&quot; AvailabilitySource=&quot;S&quot; ParticipantLevel=&quot;Secure Sell&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;O and D cache or polled status used with different local status&quot;>
               &lt;air:FlightDetails Key=&quot;TLsU9ytY1DKANGdHAAAAAA==&quot; Origin=&quot;AMS&quot; Destination=&quot;BCN&quot; DepartureTime=&quot;2021-12-10T14:25:00.000+01:00&quot; ArrivalTime=&quot;2021-12-10T16:35:00.000+01:00&quot; FlightTime=&quot;130&quot; TravelTime=&quot;130&quot; Equipment=&quot;73H&quot; DestinationTerminal=&quot;1&quot; AutomatedCheckin=&quot;false&quot;/>
               &lt;common_v50_0:SellMessage>ARRIVES BCN TERMINAL 1&lt;/common_v50_0:SellMessage>
               &lt;common_v50_0:SellMessage>ADD ADVANCE PASSENGER INFORMATION SSRS DOCA/DOCO/DOCS&lt;/common_v50_0:SellMessage>
               &lt;common_v50_0:SellMessage>PERSONAL DATA WHICH IS PROVIDED TO US IN CONNECTION&lt;/common_v50_0:SellMessage>
               &lt;common_v50_0:SellMessage>WITH YOUR TRAVEL MAY BE PASSED TO GOVERNMENT AUTHORITIES&lt;/common_v50_0:SellMessage>
               &lt;common_v50_0:SellMessage>FOR BORDER CONTROL AND AVIATION SECURITY PURPOSES&lt;/common_v50_0:SellMessage>
            &lt;/air:AirSegment>
         &lt;/air:AirSolution>
         &lt;com:CreditCard Type=&quot;VI&quot; Number=&quot;4111111111111111&quot; CVV=&quot;737&quot; ExpDate=&quot;2022-06&quot; Name=&quot;JAYA KUMAR&quot; Key=&quot;PDz8y7sw6fGfaM/wYIhwmw==&quot;>
            &lt;com:BillingAddress>
               &lt;com:AddressName>Sandy Parent&lt;/com:AddressName>
               &lt;com:Street>6901 S. Havana&lt;/com:Street>
               &lt;com:Street>Apt 2&lt;/com:Street>
               &lt;com:City>Englewood&lt;/com:City>
               &lt;com:State>CO&lt;/com:State>
               &lt;com:PostalCode>80111&lt;/com:PostalCode>
               &lt;com:Country>US&lt;/com:Country>
            &lt;/com:BillingAddress>
         &lt;/com:CreditCard>
         &lt;air:OptionalServices>
            &lt;air:OptionalService Type=&quot;PreReservedSeatAssignment&quot; TotalPrice=&quot;EUR16.00&quot; SupplierCode=&quot;KL&quot; CreateDate=&quot;2021-10-27T12:18:19.675+00:00&quot; ServiceStatus=&quot;Offered&quot; SequenceNumber=&quot;553250&quot; ServiceSubCode=&quot;0B5&quot; SSRCode=&quot;RQST&quot; IssuanceReason=&quot;A&quot; Key=&quot;TLsU9ytY1DKAejrHAAAAAA==&quot; AssessIndicator=&quot;MileageOrCurrency&quot; InclusiveOfTax=&quot;true&quot; InterlineSettlementAllowed=&quot;false&quot; GeographySpecification=&quot;Sector&quot; Source=&quot;MCE&quot; ViewableOnly=&quot;false&quot; ProviderCode=&quot;1G&quot; Quantity=&quot;1&quot; BasePrice=&quot;EUR16.00&quot; ApproximateTotalPrice=&quot;EUR16.00&quot; ApproximateBasePrice=&quot;EUR16.00&quot; Taxes=&quot;EUR0.00&quot; IsRepriceRequired=&quot;false&quot;>
               &lt;common_v50_0:ServiceData Data=&quot;6-A&quot; BookingTravelerRef=&quot;8nfO9ysY1DKASJUHAAAAAA==&quot; AirSegmentRef=&quot;TLsU9ytY1DKAMGdHAAAAAA==&quot; TravelerType=&quot;ADT&quot;/>
               &lt;common_v50_0:ServiceInfo>
                  &lt;common_v50_0:Description>CHARGEABLE SEAT&lt;/common_v50_0:Description>
               &lt;/common_v50_0:ServiceInfo>
               &lt;common_v50_0:Remark>KL 1673 - EUR16.00 PER PASSENGER CHARGEABLE SEAT&lt;/common_v50_0:Remark>
               &lt;air:EMD FulfillmentType=&quot;2&quot; AssociatedItem=&quot;Flight&quot; RefundReissueIndicator=&quot;Reuse&quot; Commissionable=&quot;false&quot; Booking=&quot;SSR&quot; FulfillmentTypeDescription=&quot;Associated to a flight coupon of a ticket&quot;/>
               &lt;air:AdditionalInfo Category=&quot;S7&quot;/>
               &lt;air:FeeApplication Code=&quot;4&quot;>Per travel&lt;/air:FeeApplication>
               &lt;air:BrandingInfo Key=&quot;1&quot; CommercialName=&quot;CHARGEABLE SEAT&quot;>
                  &lt;air:Title Type=&quot;External&quot; LanguageCode=&quot;EN&quot;>CHARGEABLE SEAT&lt;/air:Title>
                  &lt;air:AirSegmentRef Key=&quot;TLsU9ytY1DKAMGdHAAAAAA==&quot;/>
               &lt;/air:BrandingInfo>
            &lt;/air:OptionalService>
            &lt;air:OptionalService Type=&quot;Baggage&quot; TotalPrice=&quot;EUR25.00&quot; SupplierCode=&quot;KL&quot; CreateDate=&quot;2021-10-27T12:18:19.677+00:00&quot; ServiceStatus=&quot;Offered&quot; SequenceNumber=&quot;5197500&quot; ServiceSubCode=&quot;0CC&quot; SSRCode=&quot;ABAG&quot; IssuanceReason=&quot;C&quot; Key=&quot;TLsU9ytY1DKAijrHAAAAAA==&quot; AssessIndicator=&quot;MileageOrCurrency&quot; Private=&quot;true&quot; InclusiveOfTax=&quot;true&quot; InterlineSettlementAllowed=&quot;true&quot; GeographySpecification=&quot;Portion&quot; Source=&quot;MCE&quot; ViewableOnly=&quot;false&quot; TotalWeight=&quot;23KG&quot; ProviderCode=&quot;1G&quot; Quantity=&quot;1&quot; BasePrice=&quot;EUR25.00&quot; ApproximateTotalPrice=&quot;EUR25.00&quot; ApproximateBasePrice=&quot;EUR25.00&quot; Taxes=&quot;EUR0.00&quot; FirstPiece=&quot;1&quot; LastPiece=&quot;1&quot; IsRepriceRequired=&quot;false&quot;>
               &lt;common_v50_0:ServiceData BookingTravelerRef=&quot;8nfO9ysY1DKASJUHAAAAAA==&quot; AirSegmentRef=&quot;TLsU9ytY1DKAMGdHAAAAAA==&quot; TravelerType=&quot;ADT&quot;/>
               &lt;common_v50_0:ServiceInfo>
                  &lt;common_v50_0:Description>1ST ADDITIONAL BAG&lt;/common_v50_0:Description>
               &lt;/common_v50_0:ServiceInfo>
               &lt;air:EMD FulfillmentType=&quot;2&quot; AssociatedItem=&quot;Flight&quot; RefundReissueIndicator=&quot;Reuse&quot; Commissionable=&quot;false&quot; Booking=&quot;SSR&quot; FulfillmentTypeDescription=&quot;Associated to a flight coupon of a ticket&quot;/>
               &lt;air:FeeApplication Code=&quot;4&quot;>Per travel&lt;/air:FeeApplication>
               &lt;air:BrandingInfo Key=&quot;2&quot; CommercialName=&quot;1ST ADDITIONAL BAG&quot;>
                  &lt;air:Title Type=&quot;External&quot; LanguageCode=&quot;EN&quot;>1ST ADDITIONAL BAG&lt;/air:Title>
                  &lt;air:AirSegmentRef Key=&quot;TLsU9ytY1DKAMGdHAAAAAA==&quot;/>
               &lt;/air:BrandingInfo>
            &lt;/air:OptionalService>
         &lt;/air:OptionalServices>
         &lt;!--&lt;air:SpecificSeatAssignment BookingTravelerRef=&quot;462HWjtY1DKArEKvCAAAAA==&quot; SegmentRef=&quot;462HWjtY1DKAv+JvCAAAAA==&quot; SeatId=&quot;02A&quot;/>-->
         &lt;!--&lt;air:SpecificSeatAssignment BookingTravelerRef=&quot;462HWjtY1DKArEKvCAAAAA==&quot; SegmentRef=&quot;462HWjtY1DKAx+JvCAAAAA==&quot; SeatId=&quot;04A&quot;/>-->
      &lt;/univ:AirMerchandisingFulfillmentReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
