//! Jazal Goldmane — `{2}{W}{W}` 4/4 Legendary Cat Warrior with First strike.
//!
//! Oracle:
//! * First strike.
//! * "{3}{W}{W}: Attacking creatures you control get +X/+X until end of
//!   turn, where X is the number of attacking creatures." — a mana-only
//!   activated ability. X is dynamic: the number of attacking creatures
//!   on the battlefield. We compute X via `script::count_matching` over
//!   attacking creatures and apply a `Pump` to each attacker via
//!   `Effect::ForEach`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jazal Goldmane");
    let cat = reg.interner_mut().intern("Cat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{W}{W}: Attacking creatures you control get +X/+X until end of turn, where X is the number of attacking creatures.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_attackers,
        }),
    )
}

fn pump_attackers(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .attacking_only();
    let x = script::count_matching(state, &filter, ctx.controller) as i32;
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
