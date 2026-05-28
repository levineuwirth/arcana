//! Shimian Night Stalker — `{3}{B}{B}` 4/4 black Nightstalker.
//! "{B}, {T}: All damage that would be dealt to you this turn by target attacking
//! creature is dealt to this creature instead."
//! GAP: "redirect damage from player to this creature" — PreventDamage can prevent
//! damage but doesn't redirect it. No DamageRedirect variant in Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shimian Night Stalker");
    let nightstalker = reg.interner_mut().intern("Nightstalker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightstalker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}: All damage that would be dealt to you this turn by target attacking creature is dealt to this creature instead.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: redirect_damage,
            }),
    )
}

fn redirect_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "redirect damage from you to this creature" — no Effect::RedirectDamage
    // variant; PreventDamage cannot redirect, only prevent.
    Vec::new()
}
