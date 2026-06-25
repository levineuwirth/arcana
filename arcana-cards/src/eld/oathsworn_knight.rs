//! Oathsworn Knight — `{1}{B}{B}` 0/0 Human Knight.
//!
//! Oracle:
//! * This creature enters with four +1/+1 counters on it.
//! * This creature attacks each combat if able.
//! * If damage would be dealt to this creature while it has a +1/+1 counter on
//!   it, prevent that damage and remove a +1/+1 counter from it.
//!
//! * "enters with four +1/+1 counters" is an as-enters replacement with no
//!   expressible hook in this card class (no enters-with constructor in the
//!   demonstrated surface). (GAP)
//! * "attacks each combat if able" — wired as a SELF must-attack continuous
//!   effect installed on ETB.
//! * The damage-prevention / remove-counter clause is a replacement effect,
//!   not a triggered/activated ability. (GAP)

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
    let name = reg.interner_mut().intern("Oathsworn Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

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
