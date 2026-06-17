//! Merry, Esquire of Rohan — `{R}{W}` 2/2 Legendary Halfling Knight with Haste.
//! "Merry has first strike as long as it's equipped. (conditional static — GAP'd)
//!  Whenever you attack with Merry and another legendary creature, draw a card."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Merry, Esquire of Rohan");
    let halfling = reg.interner_mut().intern("Halfling");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "first strike as long as it's equipped" is a conditional continuous
    // static — not expressible as a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever you attack with Merry and another legendary
                // creature" — modeled as Merry attacking; the co-attacker
                // ("and another legendary creature") condition is a fidelity gap.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
