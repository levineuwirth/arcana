//! Quina, Qu Gourmet — `{2}{G}` 2/3 Legendary Creature — Qu.
//! "If one or more tokens would be created under your control, those
//!  tokens plus a 1/1 green Frog creature token are created instead.
//!  {2}, Sacrifice a Frog: Put a +1/+1 counter on Quina."
//!
//! The token-creation replacement static is a GAP (no usable
//! replacement-effect hook in this card class). The activated ability —
//! {2} plus sacrificing a Frog you control, to put a +1/+1 counter on
//! Quina — is fully wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quina, Qu Gourmet");
    let qu = reg.interner_mut().intern("Qu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(qu);

    let frog_filter = script::subtype_filter(reg, "Frog");

    // GAP (static replacement): "If one or more tokens would be created under
    // your control, those tokens plus a 1/1 green Frog token are created
    // instead" — token-creation replacement static, no usable hook here.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, Sacrifice a Frog: Put a +1/+1 counter on Quina.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                sacrifice_other: Some(frog_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_counter_on_self,
        }),
    )
}

fn put_counter_on_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
