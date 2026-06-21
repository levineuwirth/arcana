//! Gastal Raider — `{2}{B}` 2/1 Vampire Rogue.
//!
//! Oracle:
//! * Start your engines! (sets max-speed eligibility — NOT a modeled keyword.)
//! * When this creature enters, target opponent reveals their hand. You choose
//!   an instant or sorcery card from it. That player discards that card.
//! * Max speed — This creature gets +1/+1 and has menace.
//!
//! GAP: "Start your engines!" / "Max speed" (speed mechanic) is not a modeled
//! keyword nor a representable static — emitted as no keyword line.
//! GAP: the Max-speed static pump (+1/+1, menace) is a speed-gated continuous
//! static with no expressible representation.

use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gastal Raider");
    let vampire = reg.interner_mut().intern("Vampire");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_targeted_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn etb_targeted_discard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target opponent reveals their hand; you choose an instant or sorcery
    // card from it; that player discards that card" — coercive reveal-and-choose
    // discard (controller picks the named opponent's card) is not expressible;
    // Effect::Discard only lets the discarding player choose their own card.
    Vec::new()
}
