//! Eternal Skylord — `{4}{U}` 3/3 Zombie Wizard.
//! "When this creature enters, amass Zombies 2."

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
    let name = reg.interner_mut().intern("Eternal Skylord");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    // Interned at register so the amass resolver can look them up.
    let _army = reg.interner_mut().intern("Army");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_amass_zombies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_amass_zombies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Zombie").unwrap_or_default();
    vec![Effect::Amass {
        controller: trig.controller,
        count: 2,
        army_subtype,
        race_subtype,
    }]
}
