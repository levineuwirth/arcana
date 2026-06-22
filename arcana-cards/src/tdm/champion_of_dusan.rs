//! Champion of Dusan — `{2}{G}` 4/2 Human Warrior with Trample.
//! "Renew — {1}{G}, Exile this card from your graveyard: Put a +1/+1
//! counter and a trample counter on target creature. Activate only as a
//! sorcery."
//!
//! Trample is wired. Renew is just a flavor name for a graveyard-activated
//! ability, which is fully expressible: {1}{G} + exile-this-from-graveyard
//! cost, sorcery-speed, target creature, adding a +1/+1 counter and a
//! (named) trample counter.

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
    let name = reg.interner_mut().intern("Champion of Dusan");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    // Pre-intern the named "trample" counter so the resolver can recover
    // it via a read-only lookup.
    let _trample_counter = reg.interner_mut().intern("trample");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{G}, Exile this card from your graveyard: Put a +1/+1 counter and a trample counter on target creature. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
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

fn renew_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if let Some(trample) = reg.interner().lookup("trample") {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::Named(trample),
            count: 1,
        });
    }
    effects
}
