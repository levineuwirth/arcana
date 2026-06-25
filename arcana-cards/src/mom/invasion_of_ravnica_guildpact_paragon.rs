//! Invasion of Ravnica // Guildpact Paragon — `{5}` Battle — Siege.
//! Front (Invasion of Ravnica — Battle — Siege, enters with 5 defense):
//!   When this Siege enters, exile target nonland permanent an opponent
//!   controls that isn't exactly two colors.
//! Back (Guildpact Paragon — Artifact Creature — Construct):
//!   Whenever you cast a spell that's exactly two colors, look at the top six
//!   cards of your library. You may reveal a card that's exactly two colors
//!   from among them and put it into your hand. Put the rest on the bottom of
//!   your library in a random order.
//!
//! GAPs:
//! - ETB target restriction "that isn't exactly two colors": there is no
//!   color-count filter primitive, so the restriction is dropped — the target
//!   is any nonland permanent an opponent controls. The exile itself is modeled.
//! - Back face "Whenever you cast a spell that's exactly two colors, look at
//!   the top six..., reveal a two-color card, put it into hand": the
//!   exactly-two-colors gate has no filter primitive and the look-reveal-pick
//!   shape (DigTopN with a color-count filter) cannot express the
//!   color-count constraint, so this triggered ability is not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Ravnica");
    let siege = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Guildpact Paragon — Artifact Creature — Construct, 3/3.
    let back_name = reg.interner_mut().intern("Guildpact Paragon");
    let construct = reg.interner_mut().intern("Construct");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(construct);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            // GAP: cast-trigger dig (exactly-two-colors gate + color-count
            //   reveal filter) is not expressible; not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 5,
            })
            // ETB: exile target nonland permanent an opponent controls.
            // (Color-count restriction dropped — see header GAP.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .controlled_by(ControllerConstraint::Opponent)
                            .without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
        // GAP: back-face cast-trigger ("Whenever you cast a spell that's exactly two
        //      colors, dig 6, reveal a two-color card to hand") — the exactly-two-colors
        //      gate has no color-count filter primitive, so neither the SpellCast trigger
        //      filter nor the DigTopN reveal filter can express it. Left unwired.
    )
}

fn exile_target(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}
