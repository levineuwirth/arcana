//! Verix Bladewing — `{2}{R}{R}` 4/4 Legendary Dragon with Flying.
//! "Kicker {3}" — GAP: Kicker is not in the usable keyword surface and there is
//!   no additional-cost field; omitted.
//! "When Verix Bladewing enters, if it was kicked, create Karox Bladewing, a
//!   legendary 4/4 red Dragon creature token with flying." — the ETB trigger is
//!   wired, but its "if it was kicked" intervening-if has no kicked-state
//!   predicate in the usable surface. Firing the token unconditionally would be
//!   materially wrong (a free 4/4 without paying kicker), so the effect body is
//!   GAP'd to empty rather than creating the token.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Verix Bladewing");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_kicked_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_kicked_token(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no "was it kicked?" predicate available; do not create Karox
    // unconditionally (that would be a free 4/4 with no kicker paid).
    Vec::new()
}
