//! A-Narfi, Betrayer King — `{2}{U}{B}` 4/3 Legendary Snow Creature — Zombie Wizard.
//!
//! Oracle:
//! * Other snow and Zombie creatures you control get +1/+1. (static anthem — GAP)
//! * `{S}{S}{S}: Return Narfi, Betrayer King from your graveyard to the
//!   battlefield tapped.` — a graveyard-activated self-reanimation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Narfi, Betrayer King");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static anthem "Other snow and Zombie creatures you control get
    // +1/+1" is a continuous static, not a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{S}{S}{S}: Return Narfi, Betrayer King from your graveyard to the battlefield tapped.".into(),
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
            effect: return_self,
        }),
    )
}

fn return_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tapped" rider — the reanimated permanent is re-id'd by the zone
    // move, so the printed "enters tapped" clause is not expressible here.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
