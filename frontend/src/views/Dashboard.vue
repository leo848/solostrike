<template>
  <div>
    <v-row>
      <v-col cols="12">
        <v-card max-width="400px" min-height="200px" class="ma-4">
          <v-card-title class="text-h3 pa-8">Neues Spiel</v-card-title>
          <v-card-text>
            <v-btn block :size="80" class="rounded-pill" href="/solostrike/play">
              <v-icon :size="80">mdi-plus</v-icon>
            </v-btn>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
  export default {
    data: () => ({
      games: [
        {
          start: new Date(2023, 6, 10, 18, 29, 30).getTime(),
          time: 60_000,
          solved: 9,
          positions: [
            { fen: "2Q2bk1/p7/7p/7P/8/3b4/6r1/1n1K4 b - - 0 51", time: 3_426 },
            { fen: "4k3/r4p1q/P3nP2/b7/1pN2nK1/6Q1/RBp2r1P/1R6 b - - 3 80", time: 10_324 },
            { fen: "7r/1rB1p2p/2k2n2/1R3p2/B2b4/3NPPPp/2K5/2N5 w - - 9 43", time: null },
            { fen: "3k1B2/rB1n3q/3P4/p7/Pp2R1P1/2R4p/1b5P/6K1 w - - 23 69", time: 1_421 },
            { fen: "6r1/P1r1pRb1/6k1/2pq2p1/4p1PQ/pn2P3/P7/RK6 w - - 7 61", time: 1_421 },
            { fen: "6r1/8/p2nkP1P/1p2P3/1PP2p2/3p4/R4N1b/2B2K2 b - - 3 54", time: 1_421 },
            { fen: "2kr3B/5Q2/1P1pp1q1/3p3p/8/pb3p1P/P2K4/R2B4 w - - 6 59", time: 1_421 },
            { fen: "1bb4r/r2p2p1/2p3BP/n1p1PPk1/1N5R/8/1K6/6N1 w - - 2 60", time: 1_421 },
            { fen: "8/r1bR4/bp5k/P2R1rpp/2p1q1P1/P6N/3P4/Q4BK1 w - - 0 39", time: 1_421 },
            { fen: "2kB4/2P2Q2/4p3/1p2P3/1P1b4/5p1r/5P2/Nn2K2R b - - 1 64", time: 1_421 },,
          ]
        }
      ]
    }),
  }
</script>
