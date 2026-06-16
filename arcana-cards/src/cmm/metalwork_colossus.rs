//! Metalwork Colossus — `{11}` 10/10 Artifact Creature — Construct.
//! "This spell costs {X} less to cast, where X is the total mana value of
//! noncreature artifacts you control." (GAP — no cost-reduction primitive.)
//! "Sacrifice two artifacts: Return this card from your graveyard to your
//! hand." (activated from the graveyard)

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
    let name = reg.interner_mut().intern("Metalwork Colossus");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{11}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        ..Default::default()
    };
    // GAP: static "costs {X} less, X = total mana value of noncreature
    // artifacts you control" — no cost-reduction effect primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice two artifacts: Return this card from your graveyard to your hand.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_to_hand,
            }),
    )
}

fn return_to_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
