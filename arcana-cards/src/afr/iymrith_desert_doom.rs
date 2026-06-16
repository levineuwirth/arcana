//! Iymrith, Desert Doom — `{3}{U}{U}` 5/5 Legendary Dragon.
//! Flying.
//! Iymrith has ward {4} as long as it's untapped.
//! Whenever Iymrith deals combat damage to a player, draw a card. Then if you
//! have fewer than three cards in hand, draw cards equal to the difference.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iymrith, Desert Doom");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "ward {4} as long as it's untapped" — conditional ward is not an
    // expressible keyword/effect here.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_draw(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Draw a card, then if you have fewer than three cards in hand draw the
    // difference. After drawing one we measure hand size; the catch-up draw is
    // max(0, 3 - hand_size_after_first_draw). We compute against the current
    // hand size + 1 to approximate post-first-draw size.
    let current = script::hand_size(state, trig.controller);
    let after_first = current + 1;
    let catch_up = if after_first < 3 { 3 - after_first } else { 0 };
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1 + catch_up,
    }]
}
