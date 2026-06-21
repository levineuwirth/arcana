//! Bounteous Kirin — `{5}{G}{G}` 4/4 legendary Kirin Spirit with Flying.
//! "Whenever you cast a Spirit or Arcane spell, you may gain life equal
//! to that spell's mana value."
//!
//! The trigger (Spirit-or-Arcane spell you cast) is expressible; the
//! payoff is GAP'd because the cast spell's mana value isn't available
//! to the resolver as a dynamic amount.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bounteous Kirin");
    let kirin = reg.interner_mut().intern("Kirin");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kirin);
    subtypes.0.insert(spirit);

    let arcane_sym = reg.interner_mut().intern("Arcane");
    let spell_filter = ObjectFilter::new()
        .with_subtypes_any(vec![spirit, arcane_sym]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_life_for_mv,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_life_for_mv(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gain life equal to that spell's mana value" — no accessor for
    // the triggering cast spell's mana value, so the dynamic amount can't
    // be computed. (The "you may" is also a resolution-time choice.)
    Vec::new()
}
