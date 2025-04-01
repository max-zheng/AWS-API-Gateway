FROM gcr.io/apigee-release/hybrid/apigee-emulator:1.6.1

# Copy the OpenAPI specification
COPY apigee.yaml /opt/apigee/conf/

# Expose the emulator ports
EXPOSE 8080 8081

# Set environment variables
ENV APIGEE_EMULATOR_PORT=8080
ENV APIGEE_EMULATOR_ADMIN_PORT=8081
ENV APIGEE_EMULATOR_CONFIG=/opt/apigee/conf/apigee.yaml

# Start the emulator
CMD ["/opt/apigee/emulator/start.sh"]