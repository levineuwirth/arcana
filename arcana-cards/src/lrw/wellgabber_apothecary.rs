//! Wellgabber Apothecary — `{4}{W}` 2/3 Merfolk Cleric.
//! `{1}{W}: Prevent all damage that would be dealt to target tapped Merfolk or Kithkin creature this turn.`
//! Note: Target is "tapped Merfolk or Kithkin" — using a filter for tapped creatures with
//! either subtype. PreventDamage targets an ObjectId; amount: None prevents ALL damage.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wellgabber Apothecary");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let cleric = reg.interner_mut().intern("Cleric");
    let _kithkin = reg.interner_mut().intern("Kithkin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(cleric);
    // Build a filter for tapped Merfolk or Kithkin creatures
    let target_filter = ObjectFilter::creature()
        .tapped_only()
        .with_subtypes_any(vec![
            reg.interner_mut().intern("Merfolk"),
            reg.interner_mut().intern("Kithkin"),
        ]);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}: Prevent all damage that would be dealt to target tapped Merfolk or Kithkin creature this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(target_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_damage,
            }),
    )
}

fn prevent_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: None, // prevent ALL damage
        duration: ReplacementDuration::EndOfTurn,
    }]
}
