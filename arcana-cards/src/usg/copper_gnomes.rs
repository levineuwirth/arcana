//! Copper Gnomes — `{2}` 1/1 colorless Artifact Creature — Gnome.
//! "{4}, Sacrifice this creature: You may put an artifact card from your hand onto the battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Copper Gnomes");
    let gnome = reg.interner_mut().intern("Gnome");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}, Sacrifice this creature: You may put an artifact card from your hand onto the battlefield.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deploy_from_hand,
            }),
    )
}

fn deploy_from_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may put an artifact card from your hand onto the battlefield" —
    // optional pick over the controller's hand.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        tapped: false,
    }]
}
