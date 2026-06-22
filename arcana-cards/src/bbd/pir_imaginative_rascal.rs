//! Pir, Imaginative Rascal — `{2}{G}` 1/1 Legendary Creature — Human.
//!
//! * Partner with Toothy, Imaginary Friend (When this creature enters, target
//!   player may put Toothy into their hand from their library, then shuffle.)
//! * Static replacement: "If one or more counters would be put on a permanent
//!   your team controls, that many plus one of each of those kinds of
//!   counters are put on that permanent instead." — a continuous replacement
//!   effect, not a triggered/activated ability.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pir, Imaginative Rascal");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    // Pre-intern the partner card's name for the tutor-by-name lookup.
    let _toothy = reg.interner_mut().intern("Toothy, Imaginary Friend");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Partner with" / "Partner" are not expressible KeywordAbility
        // variants; only the Partner-with ETB tutor is modeled below.
        ..Default::default()
    };
    // GAP: replacement static "counters put on a permanent your team controls
    // → that many plus one instead" — a continuous replacement effect, not an
    // ability def.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_partner_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn etb_partner_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "target player may put Toothy into their hand from their library, then
    // shuffle." The "may" is a resolution-time choice on the tutor.
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let nm = reg.interner().lookup("Toothy, Imaginary Friend");
    vec![Effect::TutorToHand {
        player: *p,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: false,
    }]
}
