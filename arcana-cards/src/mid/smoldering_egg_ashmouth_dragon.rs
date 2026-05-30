//! Smoldering Egg // Ashmouth Dragon — `{1}{R}` Dragon Egg 0/4 with Defender (front).
//! Whenever you cast an instant or sorcery spell, put ember counters on this creature
//! equal to the mana spent to cast that spell. Then if it has 7+ ember counters,
//! remove them and transform it.
//! Back face (Ashmouth Dragon): 4/4 Dragon with Flying. Whenever you cast an instant
//! or sorcery spell, this creature deals 2 damage to any target.
//!
//! GAP: "put a number of ember counters equal to the amount of mana spent to cast that
//! spell" — the amount of mana spent is not accessible via the script API. Counter
//! addition and the threshold-transform are omitted.
//! GAP: back-face-only triggered ability (instant/sorcery cast → deal 2 damage to any
//! target) not auto-installed on transform.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smoldering Egg");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let egg_sub = reg.interner_mut().intern("Egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    subtypes.0.insert(egg_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ashmouth Dragon");
    let back_dragon_sub = reg.interner_mut().intern("Dragon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_dragon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    let instant_sorcery_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Whenever you cast an instant or sorcery spell, put ember counters
            // equal to mana spent (GAP: amount not accessible) then check threshold (GAP).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(instant_sorcery_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_instant_sorcery_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only triggered ability not auto-installed on transform.
    )
}

fn on_instant_sorcery_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put a number of ember counters equal to the amount of mana spent"
    // — mana spent to cast the triggering spell is not accessible via the script API.
    // Counter addition and threshold-based transform omitted.
    Vec::new()
}
