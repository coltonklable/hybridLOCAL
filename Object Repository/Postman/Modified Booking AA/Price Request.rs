<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Price Request</name>
   <tag></tag>
   <elementGuidId>10478c5a-d606-4904-9627-3c64565150f9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;${segment_price_request}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Identity-Key</name>
      <type>Main</type>
      <value>${Identity-Key}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/price/offers/buildfromcatalogofferings</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ts_environment</defaultValue>
      <description></description>
      <id>09e26d85-94d0-4e73-9987-1910246f5de9</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>2cf83cc0-27aa-4c65-b83c-61d8566766e4</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>2d34b819-08d3-4ae5-b9ad-aac2cde3fbb7</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>0bdbe4f1-7c8f-4d3f-8f56-1dfab3affb84</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Identity-Key</defaultValue>
      <description></description>
      <id>67b07fbd-668f-4eff-84d1-f5d0b37ec1d6</id>
      <masked>false</masked>
      <name>Identity-Key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>67cdfb6b-3127-4aaf-aa32-476c3403ab59</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.segment_price_request</defaultValue>
      <description></description>
      <id>d1108336-9bc7-4cbf-8590-8d705397e6b2</id>
      <masked>false</masked>
      <name>segment_price_request</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
