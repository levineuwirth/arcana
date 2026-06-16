//! Alacrian Jaguar — `{4}{G}` 4/4 Cat Mount.
//! Vigilance.
//! Whenever this creature attacks while saddled, it gets +2/+2 until end
//! of turn. (the "while saddled" gate is unexpressible — GAP)
//! Saddle 1. (Saddle is not a supported keyword — GAP)

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Alacrian Jaguar");
    let cat = reg.interner_mut().intern("Cat");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(mount);

    // GAP: Saddle 1 — Saddle is not a supported KeywordAbility variant and its
    // tap-other "becomes saddled" activated ability has no expressible payoff.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "while saddled" — no saddled-state predicate;
            // firing on every attack would be wrong, so the pump is omitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_while_saddled,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_while_saddled(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "while saddled" gate is not expressible; emit nothing rather than
    // pumping on every attack. (The +2/+2 itself would be Effect::Pump.)
    Vec::new()
}
