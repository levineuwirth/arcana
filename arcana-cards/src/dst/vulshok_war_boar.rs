//! Vulshok War Boar — `{2}{R}{R}` 5/5 red Creature — Boar Beast.
//! "When this creature enters, sacrifice it unless you sacrifice an artifact."
//! Wired as an optional sacrifice payment: the controller may sacrifice an
//! artifact (the payment); if they decline, Vulshok War Boar is sacrificed
//! (the penalty).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vulshok War Boar");
    let boar = reg.interner_mut().intern("Boar");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "sacrifice it unless you sacrifice an artifact" — the controller may pay
    // by sacrificing an artifact; declining sacrifices Vulshok War Boar itself
    // (the source, routed through DestroyPermanent per the "sacrifice this"
    // idiom).
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Artifact),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DestroyPermanent {
            target: trig.source,
        })),
    }]
}
