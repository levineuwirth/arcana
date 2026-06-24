//! Wyll of the Fey Pact — `{3}{R}{R}{G}` 5/5 red/green Legendary Human Warlock.
//! "When this creature specializes, you may sacrifice another creature or an
//! artifact. If you do, Wyll of the Fey Pact perpetually gets +3/+3 and gains
//! trample."
//! GAP: "specializes" trigger — no Specialize/Specialize-specific TriggerCondition
//! exists in the engine. Using SelfEntersBattlefield as a placeholder.
//! GAP: "perpetually gets +3/+3 and gains trample" — perpetual stat/keyword
//! modification is not modeled; Effect::Pump with Duration::EndOfTurn is used
//! as a best-effort substitute.
//!
//! The "you may sacrifice another creature or an artifact. If you do, …" gate is wired
//! as an optional sacrifice payment (sacrifice a creature or artifact, then pump). Minor
//! over-inclusion: the selection can't exclude the source ("another"), so Wyll itself is
//! technically offerable.

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wyll of the Fey Pact");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "when this creature specializes" has no matching
                // TriggerCondition variant; using SelfEntersBattlefield as placeholder.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_specialize,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_specialize(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may sacrifice another creature or an artifact. If you do, Wyll of
    // the Fey Pact perpetually gets +3/+3 and gains trample."
    // GAP: "perpetually" — using Duration::EndOfTurn as placeholder.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::CreatureOrArtifact),
        then: Box::new(Effect::Pump {
            target: trig.source,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample],
        }),
        else_effect: None,
    }]
}
