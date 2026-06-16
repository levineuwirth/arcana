//! Narfi, Betrayer King — `{3}{U}{B}` 4/3 Legendary Snow Zombie Wizard.
//! "Other snow and Zombie creatures you control get +1/+1."
//! "{S}{S}{S}: Return this card from your graveyard to the battlefield tapped.
//!  ({S} can be paid with one mana from a snow source.)"
//!
//! The anthem ("Other snow and Zombie creatures you control get +1/+1") is a
//! pure continuous static with no trigger/activation hook — GAP'd. The
//! graveyard-activated recursion is wired; the "tapped" rider on the return is
//! not expressible on ReturnFromGraveyardToBattlefield and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narfi, Betrayer King");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);

    // GAP: "Other snow and Zombie creatures you control get +1/+1." — a pure
    // continuous static anthem with no trigger or activation cost.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY | SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{S}{S}{S}: Return this card from your graveyard to the battlefield tapped.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{S}{S}{S}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_self_from_graveyard,
        }),
    )
}

fn return_self_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tapped" — ReturnFromGraveyardToBattlefield has no tapped rider;
    // the creature returns untapped.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
