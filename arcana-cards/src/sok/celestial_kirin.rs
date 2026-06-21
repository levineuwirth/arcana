//! Celestial Kirin — `{2}{W}{W}` 3/3 Legendary Kirin Spirit with Flying.
//! "Whenever you cast a Spirit or Arcane spell, destroy all permanents
//! with that spell's mana value."
//!
//! Flying is a base keyword. The SpellCast trigger is filtered to a
//! Spirit-or-Arcane spell (subtype OR). Its body — "destroy all
//! permanents with THAT SPELL's mana value" — needs the triggering
//! spell's mana value at resolution, for which PendingTrigger exposes no
//! accessor, so the destroy body is GAP'd to avoid a wrong literal cmc.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Celestial Kirin");
    let kirin = reg.interner_mut().intern("Kirin");
    let spirit_subtype = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kirin);
    subtypes.0.insert(spirit_subtype);

    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![spirit, arcane]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(spell_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: destroy_by_spell_mv,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn destroy_by_spell_mv(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "destroy all permanents with that spell's mana value" — the
    // triggering spell's mana value is not exposed on PendingTrigger, so the
    // exact-cmc sweep cannot be computed; emitting any literal cmc would be
    // materially wrong.
    Vec::new()
}
