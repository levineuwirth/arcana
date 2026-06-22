//! Superior Foes of Spider-Man — `{2}{R}` 3/3 Creature — Human Rogue
//! Villain. Trample.
//! "Whenever you cast a spell with mana value 4 or greater, you may
//! exile the top card of your library. If you do, you may play that
//! card until you exile another card with this creature."
//!
//! Trample is a base keyword. The cast trigger (gated to mana value 4
//! or greater) is wired as an impulse exile of the top card with
//! play-permission. The "until you exile another card with this
//! creature" custom expiry is a fidelity gap — the standard impulse
//! permission lapses at end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Superior Foes of Spider-Man");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_min_cmc(4)),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: impulse_top,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn impulse_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may exile the top card of your library; you may play it"
    // ("until you exile another card with this creature" → standard
    // impulse lapse at end of turn; custom-expiry is a fidelity gap).
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
