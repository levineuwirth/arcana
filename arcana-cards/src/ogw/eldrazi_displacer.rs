//! Eldrazi Displacer — `{2}{W}` 3/3 Eldrazi. Devoid (colorless).
//!
//! * Devoid — the card has no color (modeled by `ColorSet::colorless()`; Devoid
//!   itself is not a KeywordAbility variant).
//! * {2}{C}: Exile another target creature, then return it to the battlefield
//!   tapped under its owner's control. Modeled as exile + return-from-exile
//!   blink; the "tapped" rider on the return is a fidelity GAP (no tapped flag
//!   on ReturnFromExileToBattlefield).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eldrazi Displacer");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{C}: Exile another target creature, then return it to \
                       the battlefield tapped under its owner's control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{C}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blink_creature,
            }),
    )
}

fn blink_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // Exile, then return from exile. The "tapped" rider is a fidelity GAP.
    vec![Effect::Sequence(vec![
        Effect::ExilePermanent { target: *id },
        Effect::ReturnFromExileToBattlefield { target: *id },
    ])]
}
