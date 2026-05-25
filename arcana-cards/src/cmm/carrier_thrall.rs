//! Carrier Thrall — `{1}{B}` 2/1 Creature — Vampire.
//! "When this creature dies, create a 1/1 colorless Eldrazi Scion creature token. It has "Sacrifice this token: Add {C}.""

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Carrier Thrall");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let _scion = reg.interner_mut().intern("Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: carrier_thrall_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn carrier_thrall_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let scion_tok = reg.interner().lookup("Scion").expect("Scion interned during register()");
    let mut subtypes_0 = SubtypeSet::default();
    subtypes_0.0.insert(scion_tok);
    vec![
        Effect::CreateToken { controller: trig.controller, token: TokenDefinition { name: scion_tok, colors: ColorSet::colorless(), types: TypeLine::CREATURE.into(), subtypes: subtypes_0, power: Some(PtValue::Fixed(0)), toughness: Some(PtValue::Fixed(1)), keywords: vec![], abilities: vec![] } },
    ]
}
