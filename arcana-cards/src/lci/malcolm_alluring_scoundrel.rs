//! Malcolm, Alluring Scoundrel — `{1}{U}` 2/1 Legendary Siren Pirate.
//!
//! Flash, Flying.
//! Whenever Malcolm deals combat damage to a player, put a chorus counter
//! on it. Draw a card, then discard a card. If there are four or more
//! chorus counters on Malcolm, you may cast the discarded card without
//! paying its mana cost.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malcolm, Alluring Scoundrel");
    let siren = reg.interner_mut().intern("Siren");
    let pirate = reg.interner_mut().intern("Pirate");
    let _chorus = reg.interner_mut().intern("chorus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "this creature deals combat damage to a player" — closest faithful
            // filter is your own creatures; the source-self narrowing is not
            // expressible in DamageDealt's source_filter.
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: on_combat_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_combat_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let chorus = reg.interner().lookup("chorus");
    let mut effects = Vec::new();
    if let Some(chorus) = chorus {
        effects.push(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(chorus),
            count: 1,
        });
    }
    effects.push(Effect::DrawCards { player: trig.controller, count: 1 });
    effects.push(Effect::Discard {
        player: trig.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    });
    // GAP: "If there are four or more chorus counters on Malcolm, you may cast
    // the discarded card without paying its mana cost." — there is no Effect to
    // cast a just-discarded card for free; this rider is unexpressible.
    effects
}
