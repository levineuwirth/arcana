//! Repository Skaab — `{3}{U}` 3/3 Zombie.
//!
//! * Exploit (ETB: you may sacrifice a creature.) — Exploit has no dedicated
//!   TriggerCondition; the ETB sacrifice and the "exploits a creature" payoff
//!   are collapsed into a single ETB trigger (Sidisi pattern).
//! * "When this creature exploits a creature, return target instant or sorcery
//!   card from your graveyard to your hand." — modeled as the ETB payoff
//!   returning the targeted graveyard card.
//!
//! GAP: the "you may" optionality on the exploit-sacrifice is not expressible
//! (no optional-sacrifice cost shape), so the sacrifice is best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Repository Skaab");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exploit_then_return,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::INSTANT | TypeLine::SORCERY,
                    )),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn exploit_then_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Exploit: sacrifice a creature (best-effort; "you may" optionality GAP'd).
    let mut effects = vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }];
    // Payoff: return the targeted instant/sorcery card from graveyard to hand.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
    }
    effects
}
