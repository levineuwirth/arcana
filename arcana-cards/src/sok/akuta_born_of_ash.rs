//! Akuta, Born of Ash — `{2}{B}{B}` 3/2 Legendary Spirit with Haste.
//! "At the beginning of your upkeep, if you have more cards in hand than
//! each opponent, you may sacrifice a Swamp. If you do, return Akuta from
//! your graveyard to the battlefield."
//! - The intervening-if (more cards than EACH opponent) has no conditions
//!   helper, so it stays None and is noted.
//! - The "you may sacrifice a Swamp. If you do, return ~" body has no
//!   expressible optional-sacrifice→reanimate primitive (OptionalPaymentKind
//!   has only Mana/Life), so the effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akuta, Born of Ash");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            // GAP: intervening-if "more cards in hand than each opponent" has
            // no conditions helper (only per-self hand-size predicates exist).
            intervening_if: None,
            effect: upkeep_reanimate,
            // ability is on the card while in the graveyard.
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_reanimate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice a Swamp. If you do, return Akuta from your
    // graveyard" — optional-sacrifice-as-payment then self-reanimate is not
    // expressible (OptionalPaymentKind has only Mana/Life).
    Vec::new()
}
