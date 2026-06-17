//! Targ Nar, Demon-Fang Gnoll — `{R}{G}` 2/2 Legendary Gnoll.
//! Pack-tactics attack pump is GAP'd (no total-attacking-power predicate).
//! {2}{R}{G}: Double Targ Nar's power and toughness until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Targ Nar, Demon-Fang Gnoll");
    let gnoll = reg.interner_mut().intern("Gnoll");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnoll);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Pack tactics — "Whenever Targ Nar attacks, if you attacked with
        // creatures with total power 6 or greater this combat, attacking
        // creatures get +1/+0." No total-attacking-power intervening-if and no
        // attacking-creatures sweep pump — whole ability omitted.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{R}{G}: Double Targ Nar's power and toughness until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{R}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: double_pt,
        }),
    )
}

fn double_pt(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Double = add current power/toughness to itself until end of turn.
    let p = script::power_of(state, ctx.source).max(0);
    let t = script::toughness_of(state, ctx.source).max(0);
    vec![Effect::Pump {
        target: ctx.source,
        power: p,
        toughness: t,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
