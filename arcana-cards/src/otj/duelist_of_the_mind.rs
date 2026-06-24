//! Duelist of the Mind — `{1}{U}` */3 Creature — Human Advisor (blue).
//!
//! Oracle:
//! * Flying, vigilance — evergreen keywords.
//! * "Duelist of the Mind's power is equal to the number of cards you've drawn
//!   this turn." — an ASYMMETRIC characteristic-defining ability (power `*`,
//!   toughness fixed 3) wired at Layer 7a via a SelfEntersBattlefield
//!   `self_pt_cda`: the compute reads cards drawn this turn for power and
//!   returns the printed fixed toughness 3 on the non-`*` axis → `(drawn, 3)`.
//! * "Whenever you commit a crime, you may draw a card. If you do, discard a
//!   card. This ability triggers only once each turn." — GAP: "commit a crime"
//!   has no matching `TriggerCondition` variant.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duelist of the Mind");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA — power = cards drawn this turn; toughness fixed 3 — at 7a.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Whenever you commit a crime ..." has no matching TriggerCondition.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = cards drawn this turn by the controller; toughness is the fixed 3.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let p = script::cards_drawn_this_turn(s, who) as i32;
    (p, 3)
}
