//! Mogis's Warhound — `{1}{R}` 2/2 red Enchantment Creature — Dog.
//! Bestow {2}{R} is not in the available keyword surface and the
//! bestow alternate-cast / aura mechanic has no demonstrated primitive
//! (GAP). As modeled (no bestow / no `with_enchant`) the card is cast as
//! a creature, so the SELF clause "This creature attacks each combat if
//! able." is the live one — wired as a must-attack requirement (CR
//! 508.1a) installed on ETB. "Enchanted creature gets +2/+2 and attacks
//! each combat if able." applies only while bestowed as an Aura, which
//! is GAP'd (no bestow/aura modeling).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Mogis's Warhound");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Bestow {2}{R}" — not in the available keyword surface.
    // GAP: "Enchanted creature gets +2/+2 and attacks each combat if
    // able." — the bestowed-Aura clause; applies only while bestowed, and
    // bestow/aura is not modeled.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_must_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
