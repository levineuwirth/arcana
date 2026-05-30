//! Invasion of Innistrad // Deluge of the Dead — `{2}{B}{B}` Battle — Siege.
//!
//! Front (Battle): Flash; when this Siege enters, target creature an opponent
//! controls gets -13/-13 until end of turn.
//!
//! Back (Enchantment): When this enchantment enters, create two 2/2 black
//! Zombie creature tokens. {2}{B}: Exile target card from a graveyard. If it
//! was a creature card, create a 2/2 black Zombie creature token.
//!
//! # GAP: defeat→cast-back-face not auto-wired (CR 310.11 deferred).
//! # GAP: The back face's own ETB trigger "when this enchantment enters"
//!   is a back-face-only triggered ability; not modeled (engine debt).
//! # GAP: The back face's activated ability "{2}{B}: Exile target card from a
//!   graveyard. If it was a creature card, create a 2/2 black Zombie token."
//!   requires face-gated activated abilities on the back face, which is engine
//!   debt.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Innistrad");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // Back face: Deluge of the Dead (Enchantment)
    let back_name = reg.interner_mut().intern("Deluge of the Dead");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ENCHANTMENT.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 7,
            })
            .with_transform_back(back)
            // ETB trigger: target creature an opponent controls gets -13/-13 until end of turn
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_minus13,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_minus13(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -13,
        toughness: -13,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
