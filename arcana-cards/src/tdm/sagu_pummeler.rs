//! Sagu Pummeler — `{3}{G}` 4/4 Beast with Reach.
//! Reach
//! Renew — {4}{G}, Exile this card from your graveyard: Put two +1/+1
//!  counters and a reach counter on target creature. Activate only as a
//!  sorcery.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sagu Pummeler");
    let beast = reg.interner_mut().intern("Beast");
    let _reach_counter = reg.interner_mut().intern("reach");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Renew — {4}{G}, Exile this card from your graveyard: Put two +1/+1 counters and a reach counter on target creature. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{G}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: renew_counters,
        }),
    )
}

fn renew_counters(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut out = vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }];
    if let Some(reach) = reg.interner().lookup("reach") {
        out.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::Named(reach),
            count: 1,
        });
    }
    out
}
