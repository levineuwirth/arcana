//! Tishana, Voice of Thunder — `{5}{G}{U}` */* Legendary Merfolk Shaman.
//! "Tishana's power and toughness are each equal to the number of cards in
//!  your hand." — a characteristic-defining ability installed as a Layer 7a
//!  self-CDA on ETB via `ContinuousEffect::self_pt_cda`; PtValue::Star marks
//!  the `*/*` bones.
//! "You have no maximum hand size." — GAP: no expressible static for the
//!  maximum-hand-size rule.
//! "When Tishana enters, draw a card for each creature you control."

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tishana, Voice of Thunder");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(shaman);

    // GAP: "You have no maximum hand size." — no expressible static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: draw_per_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // CDA: P/T each equal to the number of cards in your hand.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of cards in your hand.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cards_in_hand,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = the number of cards in your hand.
fn cards_in_hand(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s.objects.objects_in_zone(Zone::Hand(who)).count() as i32;
    (n, n)
}

fn draw_per_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
